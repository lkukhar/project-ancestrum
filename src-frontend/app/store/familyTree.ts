import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Person } from '@/services/api';
import { api } from '@/services/api';

export const useFamilyTreeStore = defineStore('familyTree', () => {
  const tree = ref<Person[]>([]);
  const selectedPerson = ref<Person | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  async function loadTree() {
    isLoading.value = true;
    error.value = null;
    try {
      const familyTree = await api.getTree();
      tree.value = familyTree.persons;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load family tree';
    } finally {
      isLoading.value = false;
    }
  }

  async function addPerson(person: Omit<Person, 'id'>) {
    isLoading.value = true;
    error.value = null;
    try {
      const newPerson = await api.addPerson(person);
      tree.value.push(newPerson);
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to add person';
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function updatePerson(id: string, person: Partial<Person>) {
    isLoading.value = true;
    error.value = null;
    try {
      const updatedPerson = await api.updatePerson(id, person);
      const index = tree.value.findIndex(p => p.id === id);
      if (index !== -1) {
        tree.value[index] = updatedPerson;
      }
      if (selectedPerson.value?.id === id) {
        selectedPerson.value = updatedPerson;
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to update person';
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function deletePerson(id: string) {
    isLoading.value = true;
    error.value = null;
    try {
      await api.deletePerson(id);
      tree.value = tree.value.filter(p => p.id !== id);
      if (selectedPerson.value?.id === id) {
        selectedPerson.value = null;
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to delete person';
      throw err;
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
    selectPerson
  };
}); 