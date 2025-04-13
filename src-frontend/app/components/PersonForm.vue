<template>
  <form @submit.prevent="handleSubmit" class="person-form">
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
        v-model="formData.birth_date"
        type="date"
        placeholder="YYYY-MM-DD"
      />
    </div>

    <div class="form-group">
      <label for="deathDate">Death Date</label>
      <input
        id="deathDate"
        v-model="formData.death_date"
        type="date"
        placeholder="YYYY-MM-DD"
      />
    </div>

    <div class="form-group">
      <label for="notes">Notes</label>
      <textarea
        id="notes"
        v-model="formData.notes"
        rows="4"
        placeholder="Enter notes"
      ></textarea>
    </div>

    <div class="form-actions">
      <button type="submit" class="btn-submit">
        {{ isEditing ? 'Update' : 'Add' }} Person
      </button>
      <button type="button" class="btn-cancel" @click="$emit('cancel')">
        Cancel
      </button>
    </div>
  </form>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import type { Person } from '@/services/api';

const props = defineProps<{
  person?: Person;
}>();

const emit = defineEmits<{
  (e: 'submit', person: Omit<Person, 'id'>): void;
  (e: 'cancel'): void;
}>();

const isEditing = ref(!!props.person);

const formData = ref<Omit<Person, 'id'>>({
  name: '',
  gender: 'Male',
  birth_date: undefined,
  death_date: undefined,
  notes: '',
});

watch(
  () => props.person,
  (newPerson) => {
    if (newPerson) {
      formData.value = {
        name: newPerson.name,
        gender: newPerson.gender,
        birth_date: newPerson.birth_date,
        death_date: newPerson.death_date,
        notes: newPerson.notes,
      };
    }
  },
  { immediate: true }
);

function handleSubmit() {
  emit('submit', formData.value);
}
</script>

<style scoped>
.person-form {
  background: white;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  margin: 1rem;
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
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
}

.form-actions {
  display: flex;
  gap: 1rem;
  margin-top: 1.5rem;
}

button {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
}

.btn-submit {
  background: #4CAF50;
  color: white;
}

.btn-cancel {
  background: #f44336;
  color: white;
}
</style> 