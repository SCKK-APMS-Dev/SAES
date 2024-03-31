import { addDynamicIconSelectors } from '@iconify/tailwind';
import flowbite from 'flowbite/plugin';

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite/**/*.js'],
	theme: {
		extend: {
			fontFamily: {
				sans: ['Poppins']
			}
		}
	},
	plugins: [
		({ addVariant }) => {
			addVariant('child', '& > *');
			addVariant('child-hover', '& > *:hover');
		},
		addDynamicIconSelectors(),
		flowbite
	]
};
