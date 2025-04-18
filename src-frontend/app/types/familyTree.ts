import { Person } from './person';

export interface FamilyTree {
  persons: Person[];
  relationships: {
    from: string;
    to: string;
    type: 'Parent' | 'Child' | 'Sibling' | 'Spouse';
  }[];
} 