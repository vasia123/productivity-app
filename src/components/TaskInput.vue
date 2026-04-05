<script setup lang="ts">
import { ref } from "vue";

const emit = defineEmits<{
  create: [title: string];
}>();

const title = ref("");

function submit() {
  const val = title.value.trim();
  if (!val) return;
  emit("create", val);
  title.value = "";
}
</script>

<template>
  <div class="task-input">
    <input
      v-model="title"
      type="text"
      placeholder="New task..."
      @keyup.enter="submit"
    />
    <button class="btn-primary btn-sm" @click="submit">+</button>
  </div>
</template>

<style scoped>
.task-input {
  display: flex;
  gap: 6px;
  margin-bottom: 4px;
}

.task-input input {
  flex: 1;
  padding: 6px 10px;
  font-size: 13px;
}

.btn-sm {
  padding: 4px 10px;
  font-size: 14px;
  font-weight: bold;
}
</style>
