import { Person } from './person';

export interface Relationship {
  type: 'Parent' | 'Child' | 'Sibling' | 'Spouse';
  person: Person;
} 