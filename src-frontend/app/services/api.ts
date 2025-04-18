import { invoke } from '@tauri-apps/api'
import type { Person, Relationship, FamilyTree } from '@/types'

// Helper function to log API calls
const logApiCall = (method: string, ...args: any[]) => {
  console.debug(`API call: ${method}`, ...args)
}

export const api = {
  // Get the entire family tree
  async getTree(): Promise<FamilyTree> {
    logApiCall('getTree')
    try {
      return await invoke('get_tree')
    } catch (e) {
      console.error('Error getting tree:', e)
      return { persons: [], relationships: [] }
    }
  },

  // Add a new person
  async addPerson(person: Omit<Person, 'id'>): Promise<Person> {
    logApiCall('addPerson', person)
    try {
      return await invoke('add_person', { person })
    } catch (e) {
      console.error('Error adding person:', e)
      throw e
    }
  },

  // Get a specific person
  async getPerson(id: string): Promise<Person> {
    logApiCall('getPerson', id)
    try {
      return await invoke('get_person', { id })
    } catch (e) {
      console.error('Error getting person:', e)
      throw e
    }
  },

  // Update a person
  async updatePerson(id: string, person: Partial<Person>): Promise<Person> {
    logApiCall('updatePerson', id, person)
    try {
      return await invoke('update_person', { id, person })
    } catch (e) {
      console.error('Error updating person:', e)
      throw e
    }
  },

  // Delete a person
  async deletePerson(id: string): Promise<void> {
    logApiCall('deletePerson', id)
    try {
      return await invoke('delete_person', { id })
    } catch (e) {
      console.error('Error deleting person:', e)
      throw e
    }
  },

  async addRelationship(fromId: string, toId: string, type: Relationship['type']): Promise<void> {
    logApiCall('addRelationship', fromId, toId, type)
    try {
      return await invoke('add_relationship', { fromId, toId, type })
    } catch (e) {
      console.error('Error adding relationship:', e)
      throw e
    }
  },
}; 