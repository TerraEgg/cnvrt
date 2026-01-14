import { defineStore } from "pinia";
import { ref } from "vue";

export interface Notification {
  id: string;
  title: string;
  description?: string;
  type: "success" | "error" | "info" | "warning";
  duration: number;
  progress: number;
}

export const useNotificationStore = defineStore("notifications", () => {
  const notifications = ref<Notification[]>([]);

  const addNotification = (
    title: string,
    description?: string,
    type: "success" | "error" | "info" | "warning" = "info",
    duration = 3000
  ) => {
    const id = Math.random().toString(36);
    const notification: Notification = {
      id,
      title,
      description,
      type,
      duration,
      progress: 100,
    };

    notifications.value.push(notification);

    const startTime = Date.now();
    const progressInterval = setInterval(() => {
      const elapsed = Date.now() - startTime;
      const progress = Math.max(0, 100 - (elapsed / duration) * 100);
      
      const notif = notifications.value.find(n => n.id === id);
      if (notif) {
        notif.progress = progress;
      }

      if (progress <= 0) {
        clearInterval(progressInterval);
      }
    }, 16);

    setTimeout(() => {
      clearInterval(progressInterval);
      removeNotification(id);
    }, duration);

    return id;
  };

  const removeNotification = (id: string) => {
    notifications.value = notifications.value.filter((n) => n.id !== id);
  };

  const success = (title: string, description?: string) => {
    return addNotification(title, description, "success", 3000);
  };

  const error = (title: string, description?: string) => {
    return addNotification(title, description, "error", 4000);
  };

  const info = (title: string, description?: string) => {
    return addNotification(title, description, "info", 3000);
  };

  const warning = (title: string, description?: string) => {
    return addNotification(title, description, "warning", 3000);
  };

  return {
    notifications,
    addNotification,
    removeNotification,
    success,
    error,
    info,
    warning,
  };
});
