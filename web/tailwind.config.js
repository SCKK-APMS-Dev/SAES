import { addDynamicIconSelectors } from '@iconify/tailwind';
import flowbite from 'flowbite/plugin';

/** @type {import('tailwindcss').Config} */
export default {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite/**/*.js',
		'./node_modules/tw-elements/js/**/*.js'
	],
	theme: {
		extend: {
			fontFamily: {
				sans: ['Poppins']
			},
			colors: {
				taxi: '#fece01'
			}
		}
	},
	plugins: [
		({ addVariant }) => {
			addVariant('child', '& > *');
			addVariant('child-hover', '& > *:hover');
		},
		addDynamicIconSelectors(),
		flowbite,
		require('tw-elements/plugin.cjs')
	]
};
