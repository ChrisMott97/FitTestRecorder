import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';

type testStateType = {
  database: string,
  person: {
    id: number
  }
}

export const testState: Writable<testStateType> = writable({
  database: "",
  person: {
    id: 0
  }
});