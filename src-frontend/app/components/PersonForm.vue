<template>
  <div class="person-form">
    <h2>{{ person ? 'Edit Person' : 'Add Person' }}</h2>
    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="name">Name</label>
        <input
          id="name"
          v-model="formData.name"
          type="text"
          required
          placeholder="Enter name"
        />
      </div>

      <div class="form-group">
        <label for="gender">Gender</label>
        <select id="gender" v-model="formData.gender" required>
          <option value="Male">Male</option>
          <option value="Female">Female</option>
          <option value="Other">Other</option>
        </select>
      </div>

      <div class="form-group">
        <label for="birthDate">Birth Date</label>
        <input
          id="birthDate"
          v-model="formData.birthDate"
          type="date"
          placeholder="Birth date"
        />
      </div>

      <div class="form-group">
        <label for="deathDate">Death Date</label>
        <input
          id="deathDate"
          v-model="formData.deathDate"
          type="date"
          placeholder="Death date"
        />
      </div>

      <div class="form-group">
        <label for="notes">Notes</label>
        <textarea
          id="notes"
          v-model="formData.notes"
          placeholder="Additional notes"
        ></textarea>
      </div>

      <div class="form-actions">
        <button type="submit" class="btn-submit">
          {{ person ? 'Update' : 'Add' }}
        </button>
        <button type="button" @click="$emit('cancel')" class="btn-cancel">
          Cancel
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import type { Person } from '@/services/api'

const props = defineProps<{
  person?: Person
}>()

const emit = defineEmits<{
  (e: 'submit', person: Omit<Person, 'id'>): void
  (e: 'cancel'): void
}>()

const formData = ref<Omit<Person, 'id'>>({
  name: '',
  gender: 'Other',
  birthDate: undefined,
  deathDate: undefined,
  notes: ''
})

watch(() => props.person, (newPerson) => {
  if (newPerson) {
    formData.value = {
      name: newPerson.name,
      gender: newPerson.gender,
      birthDate: newPerson.birthDate,
      deathDate: newPerson.deathDate,
      notes: newPerson.notes
    }
  }
}, { immediate: true })

function handleSubmit() {
  emit('submit', formData.value)
}
</script>

<style scoped>
.person-form {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  max-width: 500px;
  margin: 2rem auto;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
}

.form-group input,
.form-group select,
.form-group textarea {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 1rem;
}

.form-group textarea {
  min-height: 100px;
  resize: vertical;
}

.form-actions {
  display: flex;
  gap: 0.5rem;
  margin-top: 1rem;
}

.btn-submit,
.btn-cancel {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
}

.btn-submit {
  background-color: #4CAF50;
  color: white;
  border: none;
}

.btn-cancel {
  background-color: #f44336;
  color: white;
  border: none;
}
</style> 