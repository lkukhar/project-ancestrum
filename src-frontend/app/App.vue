<template>
  <div class="app">
    <header class="app-header">
      <h1>Ancestrum - Family Tree</h1>
      <button @click="showAddForm = true" class="btn-add">Add Person</button>
    </header>

    <main class="app-content">
      <div v-if="isLoading" class="loading">Loading...</div>
      <div v-else-if="error" class="error">{{ error }}</div>
      <div v-else class="tree-view">
        <!-- Tree visualization will go here -->
        <div v-if="selectedPerson" class="selected-person">
          <PersonCard
            :person="selectedPerson"
            @edit="showEditForm = true"
            @delete="handleDelete"
          />
        </div>
      </div>
    </main>

    <PersonForm
      v-if="showAddForm"
      @submit="handleAddPerson"
      @cancel="showAddForm = false"
    />

    <PersonForm
      v-if="showEditForm && selectedPerson"
      :person="selectedPerson"
      @submit="handleUpdatePerson"
      @cancel="showEditForm = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useFamilyTreeStore } from 'store/familyTree';
import PersonCard from '@/components/PersonCard.vue';
import PersonForm from '@/components/PersonForm.vue';
import type { Person } from 'services/api';

const store = useFamilyTreeStore();
const showAddForm = ref(false);
const showEditForm = ref(false);

const { tree, selectedPerson, isLoading, error } = store;

onMounted(() => {
  store.loadTree();
});

async function handleAddPerson(person: Omit<Person, 'id'>) {
  try {
    await store.addPerson(person);
    showAddForm.value = false;
  } catch (err) {
    console.error('Failed to add person:', err);
  }
}

async function handleUpdatePerson(person: Omit<Person, 'id'>) {
  if (!selectedPerson) return;
  
  try {
    await store.updatePerson(selectedPerson.id, person);
    showEditForm.value = false;
  } catch (err) {
    console.error('Failed to update person:', err);
  }
}

async function handleDelete() {
  if (!selectedPerson) return;
  
  if (confirm('Are you sure you want to delete this person?')) {
    try {
      await store.deletePerson(selectedPerson.id);
      store.selectPerson(null);
    } catch (err) {
      console.error('Failed to delete person:', err);
    }
  }
}
</script>

<style>
.app {
  min-height: 100vh;
  background: #f5f5f5;
}

.app-header {
  background: white;
  padding: 1rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.app-header h1 {
  margin: 0;
  font-size: 1.5rem;
}

.btn-add {
  background: #4CAF50;
  color: white;
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
}

.app-content {
  padding: 1rem;
  max-width: 1200px;
  margin: 0 auto;
}

.loading,
.error {
  text-align: center;
  padding: 2rem;
  font-size: 1.2rem;
}

.error {
  color: #f44336;
}

.tree-view {
  display: grid;
  gap: 1rem;
}

.selected-person {
  max-width: 600px;
  margin: 0 auto;
}
</style> 