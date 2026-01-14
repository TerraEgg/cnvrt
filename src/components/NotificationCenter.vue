<script setup lang="ts">
import { useNotificationStore } from "../stores/notifications";

const notificationStore = useNotificationStore();
</script>

<template>
  <div class="notifications-container">
    <transition-group name="notification" tag="div">
      <div
        v-for="notification in notificationStore.notifications"
        :key="notification.id"
        :class="['notification', `notification-${notification.type}`]"
      >
        <div class="notification-content">
          <div class="notification-title">{{ notification.title }}</div>
          <div v-if="notification.description" class="notification-description">
            {{ notification.description }}
          </div>
        </div>
        <button
          class="notification-close"
          @click="notificationStore.removeNotification(notification.id)"
        >
          ✕
        </button>
        <div class="notification-countdown">
          <div
            class="notification-progress"
            :style="{ width: notification.progress + '%' }"
          ></div>
        </div>
      </div>
    </transition-group>
  </div>
</template>

<style scoped>
.notifications-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 10px;
  pointer-events: none;
}

.notification {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px 16px;
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  min-width: 300px;
  max-width: 400px;
  pointer-events: auto;
  animation: slideIn 0.3s ease;
  overflow: hidden;
  position: relative;
}

.notification-success {
  border-left: 4px solid #2ecc71;
}

.notification-error {
  border-left: 4px solid #e74c3c;
}

.notification-warning {
  border-left: 4px solid #f39c12;
}

.notification-info {
  border-left: 4px solid #3498db;
}

.notification-content {
  flex: 1;
}

.notification-title {
  font-weight: 600;
  color: #2c3e50;
  font-size: 0.95rem;
}

.notification-description {
  color: #7f8c8d;
  font-size: 0.85rem;
  margin-top: 4px;
}

.notification-close {
  background: none;
  border: none;
  color: #95a5a6;
  cursor: pointer;
  font-size: 1.2rem;
  padding: 0;
  transition: color 0.2s;
}

.notification-close:hover {
  color: #7f8c8d;
}

.notification-countdown {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: rgba(0, 0, 0, 0.1);
}

.notification-progress {
  height: 100%;
  background: currentColor;
  opacity: 0.6;
  transition: width 0.016s linear;
}

.notification-success .notification-progress {
  background: #2ecc71;
}

.notification-error .notification-progress {
  background: #e74c3c;
}

.notification-warning .notification-progress {
  background: #f39c12;
}

.notification-info .notification-progress {
  background: #3498db;
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

.notification-enter-active,
.notification-leave-active {
  transition: all 0.3s ease;
}

.notification-enter-from {
  transform: translateX(100%);
  opacity: 0;
}

.notification-leave-to {
  transform: translateX(100%);
  opacity: 0;
}
</style>
