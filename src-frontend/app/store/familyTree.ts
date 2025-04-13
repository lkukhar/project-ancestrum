import { defineStore } from 'pinia';
import { ref } from 'vue';
import { api, type Person, type FamilyTree, ApiError } from '../services/api';

export const useFamilyTreeStore = defineStore('familyTree', () => {
  const tree = ref<FamilyTree | null>(null);
  const selectedPerson = ref<Person | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  async function loadTree() {
    try {
      isLoading.value = true;
      error.value = null;
      tree.value = await api.getTree();
    } catch (err) {
      if (err instanceof ApiError) {
        error.value = `Failed to load family tree: ${err.message}`;
      } else {
        error.value = 'An unexpected error occurred';
      }
      console.error(err);
    } finally {
      isLoading.value = false;
    }
  }

  async function addPerson(person: Omit<Person, 'id'>) {
    try {
      isLoading.value = true;
      error.value = null;
      const result = await api.addPerson(person);
      await loadTree(); // Refresh the tree
      return result;
    } catch (err) {
      if (err instanceof ApiError) {
        error.value = `Failed to add person: ${err.message}`;
      } else {
        error.value = 'An unexpected error occurred';
      }
      console.error(err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function updatePerson(id: string, person: Partial<Person>) {
    try {
      isLoading.value = true;
      error.value = null;
      await api.updatePerson(id, person);
      await loadTree(); // Refresh the tree
    } catch (err) {
      if (err instanceof ApiError) {
        error.value = `Failed to update person: ${err.message}`;
      } else {
        error.value = 'An unexpected error occurred';
      }
      console.error(err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function deletePerson(id: string) {
    try {
      isLoading.value = true;
      error.value = null;
      await api.deletePerson(id);
      await loadTree(); // Refresh the tree
    } catch (err) {
      if (err instanceof ApiError) {
        error.value = `Failed to delete person: ${err.message}`;
      } else {
        error.value = 'An unexpected error occurred';
      }
      console.error(err);
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
    selectPerson,
  };
}); 