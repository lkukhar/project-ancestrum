import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Person, Relationship, FamilyTree } from '@/types';
import { api } from '@/services/api';

// Type declaration for Tauri internals
declare global {
  interface Window {
    __TAURI_INTERNALS__?: any
  }
}

// Check if running in Tauri environment
const isTauri = () => {
  return typeof window !== 'undefined' && window.__TAURI_INTERNALS__ !== undefined
}

export const useFamilyTreeStore = defineStore('familyTree', () => {
  const tree = ref<FamilyTree>({ persons: [], relationships: [] });
  const selectedPerson = ref<Person | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  const isInitialized = ref(false);

  async function loadTree() {
    if (isInitialized.value) return;
    if (!isTauri()) {
      console.debug('Not in Tauri environment, skipping tree load');
      return;
    }
    isLoading.value = true;
    error.value = null;
    try {
      tree.value = await api.getTree();
      isInitialized.value = true;
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Error loading tree';
      console.error('Error loading tree:', e);
    } finally {
      isLoading.value = false;
    }
  }

  async function addPerson(person: Omit<Person, 'id'>) {
    isLoading.value = true;
    error.value = null;
    try {
      const newPerson = await api.addPerson(person);
      tree.value.persons.push(newPerson);
      return newPerson;
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Error adding person';
      console.error('Error adding person:', e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function updatePerson(id: string, person: Partial<Person>) {
    isLoading.value = true;
    error.value = null;
    try {
      const updatedPerson = await api.updatePerson(id, person);
      const index = tree.value.persons.findIndex(p => p.id === id);
      if (index !== -1) {
        tree.value.persons[index] = updatedPerson;
      }
      if (selectedPerson.value?.id === id) {
        selectedPerson.value = updatedPerson;
      }
      return updatedPerson;
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Error updating person';
      console.error('Error updating person:', e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function deletePerson(id: string) {
    isLoading.value = true;
    error.value = null;
    try {
      await api.deletePerson(id);
      tree.value.persons = tree.value.persons.filter(p => p.id !== id);
      if (selectedPerson.value?.id === id) {
        selectedPerson.value = null;
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Error deleting person';
      console.error('Error deleting person:', e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function addRelationship(fromId: string, toId: string, type: Relationship['type']) {
    isLoading.value = true;
    error.value = null;
    try {
      await api.addRelationship(fromId, toId, type);
      tree.value.relationships.push({ from: fromId, to: toId, type });
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Error adding relationship';
      console.error('Error adding relationship:', e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  function selectPerson(person: Person | null) {
    selectedPerson.value = person;
  }

  return {
    tree,
    selectedPerson,
    isLoading,
    error,
    loadTree,
    addPerson,
    updatePerson,
    deletePerson,
    addRelationship,
    selectPerson
  };
}); 