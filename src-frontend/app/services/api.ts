import { invoke } from '@tauri-apps/api/tauri';

const API_BASE_URL = 'http://localhost:8000/api';

export interface Person {
  id: string;
  name: string;
  birthDate?: string;
  deathDate?: string;
  gender: 'Male' | 'Female' | 'Other';
  notes: string;
}

export interface Relationship {
  type: 'Parent' | 'Child' | 'Sibling' | 'Spouse';
  person: Person;
}

export interface FamilyTree {
  persons: Person[];
  relationships: {
    from: string;
    to: string;
    type: 'Parent' | 'Child' | 'Sibling' | 'Spouse';
  }[];
}

export interface ApiResponse<T> {
  data: T;
  message: string;
}

export interface ApiError {
  message: string;
  status: number;
}

export const api = {
  // Get the entire family tree
  async getTree(): Promise<FamilyTree> {
    try {
      const response = await invoke<ApiResponse<FamilyTree>>('get_tree');
      return response.data;
    } catch (error: any) {
      throw {
        message: error.message,
        status: error.status || 500,
        data: error.data
      } as ApiError;
    }
  },

  // Add a new person
  async addPerson(person: Omit<Person, 'id'>): Promise<Person> {
    try {
      const response = await invoke<ApiResponse<Person>>('add_person', { person });
      return response.data;
    } catch (error: any) {
      throw {
        message: error.message,
        status: error.status || 500,
        data: error.data
      } as ApiError;
    }
  },

  // Get a specific person
  async getPerson(id: string): Promise<Person> {
    try {
      const response = await invoke<ApiResponse<Person>>('get_person', { id });
      return response.data;
    } catch (error: any) {
      throw {
        message: error.message,
        status: error.status || 500,
        data: error.data
      } as ApiError;
    }
  },

  // Update a person
  async updatePerson(id: string, person: Partial<Person>): Promise<Person> {
    try {
      const response = await invoke<ApiResponse<Person>>('update_person', { id, person });
      return response.data;
    } catch (error: any) {
      throw {
        message: error.message,
        status: error.status || 500,
        data: error.data
      } as ApiError;
    }
  },

  // Delete a person
  async deletePerson(id: string): Promise<void> {
    try {
      await invoke<ApiResponse<void>>('delete_person', { id });
    } catch (error: any) {
      throw {
        message: error.message,
        status: error.status || 500,
        data: error.data
      } as ApiError;
    }
  },

  async addRelationship(fromId: string, toId: string, type: Relationship['type']): Promise<void> {
    try {
      await invoke<ApiResponse<void>>('add_relationship', { fromId, toId, type });
    } catch (error: any) {
      throw {
        message: error.message,
        status: error.status || 500,
        data: error.data
      } as ApiError;
    }
  },
}; 