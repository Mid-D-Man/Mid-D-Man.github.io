// Service Worker for GitHub Pages WASM MIME type fix
// This solves the critical issue where GitHub Pages doesn't serve .wasm files with correct MIME types

const CACHE_NAME = 'midmanstudio-v1';
const WASM_MIME_TYPE = 'application/wasm';

// Install event
self.addEventListener('install', event => {
    console.log('MidManStudio Service Worker: Installing...');
    self.skipWaiting();
});

// Activate event
self.addEventListener('activate', event => {
    console.log('MidManStudio Service Worker: Activated');
    event.waitUntil(self.clients.claim());
});

// Fetch event - This is the critical fix for WASM files
self.addEventListener('fetch', event => {
    const url = new URL(event.request.url);
    
    // Handle WASM files specifically
    if (url.pathname.endsWith('.wasm')) {
        console.log('Service Worker: Intercepting WASM request:', url.pathname);
        
        event.respondWith(
            fetch(event.request)
                .then(response => {
                    // Clone the response because it can only be consumed once
                    const responseClone = response.clone();
                    
                    // Check if the response is successful
                    if (response.ok) {
                        // Create a new response with the correct MIME type
                        return response.blob().then(blob => {
                            return new Response(blob, {
                                status: response.status,
                                statusText: response.statusText,
                                headers: {
                                    ...Object.fromEntries(response.headers.entries()),
                                    'Content-Type': WASM_MIME_TYPE,
                                    'Cross-Origin-Embedder-Policy': 'require-corp',
                                    'Cross-Origin-Opener-Policy': 'same-origin'
                                }
                            });
                        });
                    }
                    
                    // If response is not ok, return it as-is
                    return response;
                })
                .catch(error => {
                    console.error('Service Worker: WASM fetch failed:', error);
                    // Return a network error response
                    return new Response('WASM file not found', { 
                        status: 404, 
                        statusText: 'Not Found' 
                    });
                })
        );
        return;
    }
    
    // Handle JavaScript files (ensure proper MIME type)
    if (url.pathname.endsWith('.js')) {
        event.respondWith(
            fetch(event.request)
                .then(response => {
                    if (response.ok && !response.headers.get('content-type')?.includes('javascript')) {
                        return response.blob().then(blob => {
                            return new Response(blob, {
                                status: response.status,
                                statusText: response.statusText,
                                headers: {
                                    ...Object.fromEntries(response.headers.entries()),
                                    'Content-Type': 'application/javascript'
                                }
                            });
                        });
                    }
                    return response;
                })
                .catch(error => {
                    console.error('Service Worker: JS fetch failed:', error);
                    return fetch(event.request);
                })
        );
        return;
    }
    
    // For all other requests, use normal fetch
    // but add some helpful headers for debugging
    if (url.pathname.includes('midmanstudio')) {
        event.respondWith(
            fetch(event.request)
                .then(response => {
                    console.log(`Service Worker: ${event.request.method} ${url.pathname} - ${response.status}`);
                    return response;
                })
                .catch(error => {
                    console.error('Service Worker: Fetch failed:', error);
                    return fetch(event.request);
                })
        );
        return;
    }
    
    // Default: let the request pass through normally
});
