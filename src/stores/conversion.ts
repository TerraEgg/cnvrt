import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useNotificationStore } from "./notifications";

export interface FileItem {
  id: string;
  name: string;
  path: string;
  size: number;
  type: string;
  extension: string;
  targetFormat: string;
  file?: File;
  preview?: string;
}

interface HistoryItem {
  id: string;
  fromFormat: string;
  toFormat: string;
  success: boolean;
  timestamp: number;
}

export const useConversionStore = defineStore("conversion", () => {
  const files = ref<FileItem[]>([]);
  const history = ref<HistoryItem[]>([]);
  const isProcessing = ref(false);
  const currentProgress = ref({
    percentage: 0,
    message: "Starting conversion...",
  });

  const addFile = (file: FileItem) => {
    files.value.push(file);
  };

  const removeFile = (fileId: string) => {
    files.value = files.value.filter((f) => f.id !== fileId);
  };

  const clearFiles = () => {
    files.value = [];
  };

  const updateFileFormat = (fileId: string, format: string) => {
    const file = files.value.find((f) => f.id === fileId);
    if (file) {
      file.targetFormat = format;
    }
  };

  const addToHistory = (item: HistoryItem) => {
    history.value.unshift(item);
    if (history.value.length > 50) {
      history.value.pop();
    }
  };

  const startConversion = async (destinationFolder: string = "", keepTransparency: boolean = true) => {
    if (files.value.some((f) => !f.targetFormat)) {
      useNotificationStore().error("Missing Formats", "Please select target format for all files");
      return;
    }

    const sameFormatFiles = files.value.filter((f) => f.extension.toLowerCase() === f.targetFormat.toLowerCase());
    if (sameFormatFiles.length > 0) {
      useNotificationStore().error("Invalid Conversion", `Cannot convert ${sameFormatFiles[0].name} to the same format (${sameFormatFiles[0].targetFormat})`);
      return;
    }

    isProcessing.value = true;
    const totalFiles = files.value.length;
    const notificationStore = useNotificationStore();
    let successCount = 0;
    let failureCount = 0;

    for (let i = 0; i < files.value.length; i++) {
      const file = files.value[i];
      const isVideoFile = ['mp4', 'mkv', 'mov', 'webm', 'avi', 'flv', 'mpg', 'mpeg', 'ts', 'm2ts', 'mts', 'ogv', 'ogg'].includes(file.extension.toLowerCase());
      
      currentProgress.value = {
        percentage: 0,
        message: `Processing ${i + 1}/${totalFiles}: ${file.name}... (0%)`,
      };

      try {
        const outputName = `${file.name.split(".")[0]}.${file.targetFormat}`;
        let outputPath = outputName;
        
        if (destinationFolder) {
          outputPath = `${destinationFolder}/${outputName}`;
        }

        let inputPath = file.path;
        
        if (file.file && !file.path.includes(':') && !file.path.startsWith('/')) {
          const fileData = await file.file.arrayBuffer();
          if (!fileData) {
            throw new Error("Failed to read file data");
          }
          const result: any = await invoke("convert_image", {
            inputData: Array.from(new Uint8Array(fileData)),
            outputPath: outputPath,
            fromFormat: file.extension,
            toFormat: file.targetFormat,
            keepTransparency: keepTransparency,
          });
          
          currentProgress.value = {
            percentage: 100,
            message: `✓ Converted ${i + 1}/${totalFiles}: ${file.name}`,
          };
          
          if (result.success) {
            successCount++;
            addToHistory({
              id: result.id,
              fromFormat: file.extension.toUpperCase(),
              toFormat: file.targetFormat.toUpperCase(),
              success: true,
              timestamp: Date.now(),
            });
          } else {
            throw new Error(result.error || "Conversion failed");
          }
        } else {
          if (isVideoFile) {
            let fakeProgress = 0;
            const progressInterval = setInterval(() => {
              fakeProgress += Math.random() * 15;
              if (fakeProgress > 95) fakeProgress = 95;
              
              currentProgress.value = {
                percentage: Math.round(fakeProgress),
                message: `Processing ${i + 1}/${totalFiles}: ${file.name}... (${Math.round(fakeProgress)}%)`,
              };
            }, 500);

            const result: any = await invoke("convert_from_path", {
              inputPath: inputPath,
              outputPath: outputPath,
              fromFormat: file.extension,
              toFormat: file.targetFormat,
            });

            clearInterval(progressInterval);

            if (result.success) {
              successCount++;
              currentProgress.value = {
                percentage: 100,
                message: `✓ Converted ${i + 1}/${totalFiles}: ${file.name}`,
              };
              addToHistory({
                id: result.id,
                fromFormat: file.extension.toUpperCase(),
                toFormat: file.targetFormat.toUpperCase(),
                success: true,
                timestamp: Date.now(),
              });
            } else {
              throw new Error(result.error || "Video conversion failed");
            }
          } else {
            currentProgress.value = {
              percentage: 50,
              message: `Processing ${i + 1}/${totalFiles}: ${file.name}... (50%)`,
            };

            const result: any = await invoke("convert_from_path", {
              inputPath: inputPath,
              outputPath: outputPath,
              fromFormat: file.extension,
              toFormat: file.targetFormat,
            });

            if (result.success) {
              successCount++;
              currentProgress.value = {
                percentage: 100,
                message: `✓ Converted ${i + 1}/${totalFiles}: ${file.name}`,
              };
              addToHistory({
                id: result.id,
                fromFormat: file.extension.toUpperCase(),
                toFormat: file.targetFormat.toUpperCase(),
                success: true,
                timestamp: Date.now(),
              });
            } else {
              throw new Error(result.error || "Image conversion failed");
            }
          }
        }
      } catch (error: any) {
        failureCount++;
        console.error("Conversion error:", error);
        notificationStore.error(
          `Failed to load: ${file.name}`,
          "The file is likely corrupted"
        );
        addToHistory({
          id: Math.random().toString(36),
          fromFormat: file.extension.toUpperCase(),
          toFormat: file.targetFormat.toUpperCase(),
          success: false,
          timestamp: Date.now(),
        });
      }
    }

    isProcessing.value = false;
    
    clearFiles();
    currentProgress.value = {
      percentage: 0,
      message: "Starting conversion...",
    };

    if (failureCount === 0 && successCount > 0) {
      notificationStore.success(
        "Conversion Complete",
        `Successfully converted ${successCount} file${successCount !== 1 ? "s" : ""}`
      );
    }
  };

  return {
    files,
    history,
    isProcessing,
    currentProgress,
    addFile,
    removeFile,
    clearFiles,
    updateFileFormat,
    addToHistory,
    startConversion,
  };
});
