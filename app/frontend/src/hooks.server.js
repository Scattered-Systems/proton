

/** @type {import('@sveltejs/kit').Handle} */
export function handle({ event, resolve }) {
	const addr = event.cookies.get('address');
	event.locals.user = addr ? JSON.parse(atob(addr)) : null;
	// const jwt = event.cookies.get('jwt');
	// event.locals.user = jwt ? JSON.parse(atob(jwt)) : null;

	return resolve(event);
}