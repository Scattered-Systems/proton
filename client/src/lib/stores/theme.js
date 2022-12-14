import { writable } from 'svelte/store';

export function createTheme() {
    const props = (s) => {
        mode: s
    }
    const { subscribe, set, update } = writable(props('dark'));

    

    return {
        subscribe,
        set: () => update(n => props(n)),
        reset: () => set(props('dark'))
    };
}

export const theme = createTheme();