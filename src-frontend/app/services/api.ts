import { invoke } from '@tauri-apps/api/tauri'
import type { Person, Relationship, FamilyTree } from '@/types'

// Check if we're running in a Tauri environment
const isTauri = () => {
  try {
    return typeof window !== 'undefined' && 
           window.__TAURI__ !== undefined &&
           typeof window.__TAURI__ === 'object'
  } catch (e) {
    console.debug('Tauri API not available:', e)
    return false
  }
}

// Helper function to log API calls
const logApiCall = (method: string, ...args: any[]) => {
  console.debug(`API call: ${method}`, ...args)
}

export const api = {
  // Get the entire family tree
  async getTree(): Promise<FamilyTree> {
    logApiCall('getTree')
    if (!isTauri()) {
      console.warn('Tauri API not available - running in browser mode')
      return { persons: [], relationships: [] }
    }
    return invoke('get_tree')
  },

  // Add a new person
  async addPerson(person: Omit<Person, 'id'>): Promise<Person> {
    logApiCall('addPerson', person)
    if (!isTauri()) {
      console.warn('Tauri API not available - running in browser mode')
      return { ...person, id: crypto.randomUUID() }
    }
    return invoke('add_person', { person })
  },

  // Get a specific person
  async getPerson(id: string): Promise<Person> {
    logApiCall('getPerson', id)
    if (!isTauri()) {
      console.warn('Tauri API not available - running in browser mode')
      throw new Error('Person not found')
    }
    return invoke('get_person', { id })
  },

  // Update a person
  async updatePerson(id: string, person: Partial<Person>): Promise<Person> {
    logApiCall('updatePerson', id, person)
    if (!isTauri()) {
      console.warn('Tauri API not available - running in browser mode')
      throw new Error('Cannot update person in browser mode')
    }
    return invoke('update_person', { id, person })
  },

  // Delete a person
  async deletePerson(id: string): Promise<void> {
    logApiCall('deletePerson', id)
    if (!isTauri()) {
      console.warn('Tauri API not available - running in browser mode')
      return
    }
    return invoke('delete_person', { id })
  },

  async addRelationship(fromId: string, toId: string, type: Relationship['type']): Promise<void> {
    logApiCall('addRelationship', fromId, toId, type)
    if (!isTauri()) {
      console.warn('Tauri API not available - running in browser mode')
      return
    }
    return invoke('add_relationship', { fromId, toId, type })
  },
}; 