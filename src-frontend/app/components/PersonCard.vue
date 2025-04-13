<template>
  <div v-if="person" class="person-card">
    <div class="person-header">
      <h2>{{ person.name }}</h2>
      <div class="person-actions">
        <button @click="$emit('edit')" class="btn-edit">Edit</button>
        <button @click="$emit('delete')" class="btn-delete">Delete</button>
      </div>
    </div>
    
    <div class="person-details">
      <div class="detail">
        <label>Gender:</label>
        <span>{{ person.gender }}</span>
      </div>
      
      <div class="detail">
        <label>Birth Date:</label>
        <span>{{ person.birth_date || 'Unknown' }}</span>
      </div>
      
      <div class="detail">
        <label>Death Date:</label>
        <span>{{ person.death_date || 'Unknown' }}</span>
      </div>
      
      <div class="detail notes">
        <label>Notes:</label>
        <p>{{ person.notes }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import type { Person } from '@/services/api';

defineProps<{
  person: Person;
}>();

defineEmits<{
  (e: 'edit'): void;
  (e: 'delete'): void;
}>();
</script>

<style scoped>
.person-card {
  background: white;
  border-radius: 8px;
  padding: 1rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  margin: 1rem;
}

.person-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.person-header h2 {
  margin: 0;
  font-size: 1.5rem;
}

.person-actions {
  display: flex;
  gap: 0.5rem;
}

button {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
}

.btn-edit {
  background: #4CAF50;
  color: white;
}

.btn-delete {
  background: #f44336;
  color: white;
}

.person-details {
  display: grid;
  gap: 0.5rem;
}

.detail {
  display: flex;
  gap: 0.5rem;
}

.detail label {
  font-weight: 500;
  min-width: 100px;
}

.notes p {
  margin: 0;
  white-space: pre-wrap;
}
</style> 