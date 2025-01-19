/** @type {import('tailwindcss').Config} */
export default {
  content: ["src/**/*.svelte","src/app.html"],
  theme: {
    extend: {
			fontFamily: {
				sans: ['Poppins'],
				itim: ['Itim']
			},
			colors: {
				taxi: '#fece01',
				tow: '#3498db'
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
  plugins: [],
}

