import type { Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {
			colors: {
				text: 'var(--text)',
				background: 'var(--background)',
				primary: 'var(--primary)',
				secondary: 'var(--secondary)',
				accent: 'var(--accent)'
			}
		}
	},

	//safelist: [{
	//	pattern: new RegExp("."),
    //}],

	darkMode: 'class',
	plugins: [require('@tailwindcss/typography')]
} as Config;
