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
			},
			backgroundSize: {
				'size-200': '200% 200%'
			},
			backgroundPosition: {
				'pos-0': '0% 0%',
				'pos-100': '100% 100%'
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
