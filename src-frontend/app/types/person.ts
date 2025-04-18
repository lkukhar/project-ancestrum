export interface Person {
  id: string;
  name: string;
  birthDate?: string;
  deathDate?: string;
  gender: 'Male' | 'Female' | 'Other';
  notes: string;
} 