/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * Workbench layout parts — unit tests (vitest + jsdom).
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn().mockResolvedValue([
		{ id: 'explorer', name: 'Explorer', icon: 'folder' },
		{ id: 'search', name: 'Search', icon: 'search' },
	]),
}));

import { ActivityBar } from '../parts/activityBar.js';
import { Sidebar } from '../parts/sidebar.js';
import { Panel } from '../parts/panel.js';
import { StatusBar } from '../parts/statusBar.js';
import { TitleBar } from '../parts/titleBar.js';
import { EditorArea } from '../parts/editorArea.js';
import { Disposable } from '../lifecycle.js';

describe('Disposable base', () => {
	it('exposes a dispose() method', () => {
		const d = new Disposable();
		expect(typeof d.dispose).toBe('function');
		d.dispose();
	});

	it('isDisposed returns true after dispose()', () => {
		const d = new Disposable();
		expect(d.isDisposed).toBe(false);
		d.dispose();
		expect(d.isDisposed).toBe(true);
	});
});

describe('ActivityBar', () => {
	let host: HTMLElement;
	let part: ActivityBar;

	beforeEach(() => {
		host = document.createElement('div');
		document.body.appendChild(host);
		part = new ActivityBar();
	});

	afterEach(() => {
		part.dispose();
		document.body.removeChild(host);
	});

	it('mount() creates element with data-part="activitybar"', () => {
		part.mount(host);
		expect(host.querySelector('[data-part="activitybar"]')).not.toBeNull();
	});

	it('mount() appends child to host', () => {
		part.mount(host);
		expect(host.children.length).toBeGreaterThan(0);
	});

	it('dispose() removes mounted element', () => {
		part.mount(host);
		part.dispose();
		expect(host.querySelector('[data-part="activitybar"]')).toBeNull();
	});

	it('fetches view containers via invoke on mount', async () => {
		const { invoke } = await import('@tauri-apps/api/core');
		part.mount(host);
		await vi.waitFor(() => {
			expect(invoke).toHaveBeenCalledWith('workbench_view_containers');
		});
	});
});

describe('Sidebar', () => {
	let host: HTMLElement;
	let part: Sidebar;

	beforeEach(() => {
		host = document.createElement('div');
		document.body.appendChild(host);
		part = new Sidebar();
	});

	afterEach(() => {
		part.dispose();
		document.body.removeChild(host);
	});

	it('mount() creates element with data-part="sidebar"', () => {
		part.mount(host);
		expect(host.querySelector('[data-part="sidebar"]')).not.toBeNull();
	});

	it('dispose() removes mounted element', () => {
		part.mount(host);
		part.dispose();
		expect(host.querySelector('[data-part="sidebar"]')).toBeNull();
	});
});

describe('Panel', () => {
	let host: HTMLElement;
	let part: Panel;

	beforeEach(() => {
		host = document.createElement('div');
		document.body.appendChild(host);
		part = new Panel();
	});

	afterEach(() => {
		part.dispose();
		document.body.removeChild(host);
	});

	it('mount() creates element with data-part="panel"', () => {
		part.mount(host);
		expect(host.querySelector('[data-part="panel"]')).not.toBeNull();
	});

	it('dispose() removes mounted element', () => {
		part.mount(host);
		part.dispose();
		expect(host.querySelector('[data-part="panel"]')).toBeNull();
	});
});

describe('StatusBar', () => {
	let host: HTMLElement;
	let part: StatusBar;

	beforeEach(() => {
		host = document.createElement('div');
		document.body.appendChild(host);
		part = new StatusBar();
	});

	afterEach(() => {
		part.dispose();
		document.body.removeChild(host);
	});

	it('mount() creates element with data-part="statusbar"', () => {
		part.mount(host);
		expect(host.querySelector('[data-part="statusbar"]')).not.toBeNull();
	});

	it('dispose() removes mounted element', () => {
		part.mount(host);
		part.dispose();
		expect(host.querySelector('[data-part="statusbar"]')).toBeNull();
	});
});

describe('TitleBar', () => {
	let host: HTMLElement;
	let part: TitleBar;

	beforeEach(() => {
		host = document.createElement('div');
		document.body.appendChild(host);
		part = new TitleBar();
	});

	afterEach(() => {
		part.dispose();
		document.body.removeChild(host);
	});

	it('mount() creates element with data-part="titlebar"', () => {
		part.mount(host);
		expect(host.querySelector('[data-part="titlebar"]')).not.toBeNull();
	});

	it('dispose() removes mounted element', () => {
		part.mount(host);
		part.dispose();
		expect(host.querySelector('[data-part="titlebar"]')).toBeNull();
	});
});

describe('EditorArea', () => {
	let host: HTMLElement;
	let part: EditorArea;

	beforeEach(() => {
		host = document.createElement('div');
		document.body.appendChild(host);
		part = new EditorArea();
	});

	afterEach(() => {
		part.dispose();
		document.body.removeChild(host);
	});

	it('mount() creates element with data-part="editor"', () => {
		part.mount(host);
		expect(host.querySelector('[data-part="editor"]')).not.toBeNull();
	});

	it('dispose() removes mounted element', () => {
		part.mount(host);
		part.dispose();
		expect(host.querySelector('[data-part="editor"]')).toBeNull();
	});
});
