import { dev } from '$app/environment';
 
navigator.serviceWorker.register('/service-worker.js', {
  type: dev ? 'module' : 'classic'
});
