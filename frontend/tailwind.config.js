/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	safelist: [
		// Trust level badge classes that are dynamically generated
		'retro-trust-high',
		'retro-trust-medium-high',
		'retro-trust-medium',
		'retro-trust-low',
		'retro-trust-unknown'
	],
	theme: {
		extend: {
			colors: {
				retro: {
					bg: '#f8f8f8',
					text: '#2d2d2d',
					border: '#c0c0c0',
					highlight: '#e8e8e8',
					accent: '#7b2d8e',
					link: '#4a1a5c',
					muted: '#666666',
					green: '#b8d62f',
					amber: '#b8860b',
					orange: '#ff8c42',
					purple: '#7b2d8e',
					lime: '#b8d62f',
					error: '#fee2e2',
					'error-text': '#991b1b',
					'error-border': '#fecaca'
				}
			},
			fontFamily: {
				mono: ['JetBrains Mono', 'monospace']
			},
			fontSize: {
				'retro-xs': '10px',
				'retro-sm': '11px',
				'retro-base': '12px',
				'retro-lg': '13px',
				'retro-xl': '15px',
				'retro-2xl': '23px'
			},
			spacing: {
				'retro-xs': '1px',
				'retro-sm': '2px',
				'retro-md': '4px',
				'retro-lg': '6px',
				'retro-xl': '8px',
				'retro-2xl': '10px',
				'retro-3xl': '12px',
				'retro-4xl': '16px',
				'retro-5xl': '24px'
			},
			borderRadius: {
				retro: '4px',
				'retro-sm': '2px',
				'retro-lg': '8px'
			}
		}
	},
	plugins: []
};
