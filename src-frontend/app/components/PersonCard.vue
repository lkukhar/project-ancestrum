<template>
  <div class="person-card">
    <h2>{{ person.name }}</h2>
    <div class="person-details">
      <p v-if="person.birthDate">Born: {{ person.birthDate }}</p>
      <p v-if="person.deathDate">Died: {{ person.deathDate }}</p>
      <p>Gender: {{ person.gender }}</p>
      <p v-if="person.notes">{{ person.notes }}</p>
    </div>
    <div class="person-actions">
      <button @click="$emit('edit')" class="btn-edit">Edit</button>
      <button @click="$emit('delete')" class="btn-delete">Delete</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Person } from '@/services/api'

defineProps<{
  person: Person
}>()

defineEmits<{
  (e: 'edit'): void
  (e: 'delete'): void
}>()
</script>

<style scoped>
.person-card {
  border: 1px solid #ccc;
  border-radius: 8px;
  padding: 1rem;
  margin: 1rem;
  background-color: white;
}

.person-details {
  margin: 1rem 0;
}

.person-actions {
  display: flex;
  gap: 0.5rem;
  margin-top: 1rem;
}

.btn-edit, .btn-delete {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
}

.btn-edit {
  background-color: #4CAF50;
  color: white;
  border: none;
}

.btn-delete {
  background-color: #f44336;
  color: white;
  border: none;
}
</style> 