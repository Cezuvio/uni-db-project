import { writable } from "svelte/store";

export const warnings = writable<string[]>([]);

const _services = writable({
  table: "http://localhost:8080",
  auth: "http://localhost:8081",
  row: "http://localhost:8082",
});

export const services = {
  subscribe: _services.subscribe,
};
