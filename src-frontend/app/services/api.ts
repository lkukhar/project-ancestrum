import axios from 'axios';

const API_BASE_URL = 'http://localhost:8000/api';

export interface Person {
  id: string;
  name: string;
  birth_date?: string;
  death_date?: string;
  gender: 'Male' | 'Female' | 'Other';
  notes: string;
}

export interface Relationship {
  type: 'Parent' | 'Child' | 'Sibling' | 'Spouse';
  person: Person;
}

export interface FamilyTree {
  root: Person;
  relationships: Map<string, Relationship[]>;
}

export class ApiError extends Error {
  constructor(
    message: string,
    public status?: number,
    public data?: unknown
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

export const api = {
  // Get the entire family tree
  async getTree(): Promise<FamilyTree> {
    try {
      const response = await axios.get<FamilyTree>(`${API_BASE_URL}/tree`);
      return response.data;
    } catch (error: any) {
      throw new ApiError(
        error.message,
        error.response?.status,
        error.response?.data
      );
    }
  },

  // Add a new person
  async addPerson(person: Omit<Person, 'id'>): Promise<Person> {
    try {
      const response = await axios.post<Person>(`${API_BASE_URL}/person`, person);
      return response.data;
    } catch (error: any) {
      throw new ApiError(
        error.message,
        error.response?.status,
        error.response?.data
      );
    }
  },

  // Get a specific person
  async getPerson(id: string): Promise<Person> {
    try {
      const response = await axios.get<Person>(`${API_BASE_URL}/person/${id}`);
      return response.data;
    } catch (error: any) {
      throw new ApiError(
        error.message,
        error.response?.status,
        error.response?.data
      );
    }
  },

  // Update a person
  async updatePerson(id: string, person: Partial<Person>): Promise<Person> {
    try {
      const response = await axios.put<Person>(`${API_BASE_URL}/person/${id}`, person);
      return response.data;
    } catch (error: any) {
      throw new ApiError(
        error.message,
        error.response?.status,
        error.response?.data
      );
    }
  },

  // Delete a person
  async deletePerson(id: string): Promise<void> {
    try {
      await axios.delete(`${API_BASE_URL}/person/${id}`);
    } catch (error: any) {
      throw new ApiError(
        error.message,
        error.response?.status,
        error.response?.data
      );
    }
  },
}; 