import type { Socket } from 'socket.io-client';
import { writable, type Writable } from 'svelte/store';

export const socket: Writable<null | Socket> = writable(null);
