/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * activityBar.ts — left vertical activity bar part.
 *
 * Mirrors src/vs/workbench/browser/parts/activitybar/activitybarPart.ts.
 * Fetches view container registry from Rust via invoke('workbench_view_containers').
 */

import { invoke } from '@tauri-apps/api/core';
import { Disposable } from '../lifecycle.js';
import type { ViewContainer } from '../types.js';

export class ActivityBar extends Disposable {
	private _element: HTMLElement | null = null;

	mount(host: HTMLElement): void {
		const el = document.createElement('div');
		el.setAttribute('data-part', 'activitybar');
		el.setAttribute('role', 'navigation');
		el.setAttribute('aria-label', 'Activity Bar');
		host.appendChild(el);
		this._element = el;

		this._register({
			dispose: () => {
				el.remove();
				this._element = null;
			},
		});

		this._loadViewContainers(el);
	}

	private _loadViewContainers(container: HTMLElement): void {
		invoke<ViewContainer[]>('workbench_view_containers')
			.then((containers) => {
				if (this.isDisposed) {
					return;
				}
				for (const vc of containers) {
					const item = document.createElement('div');
					item.setAttribute('data-container-id', vc.id);
					item.setAttribute('title', vc.name);
					item.setAttribute('aria-label', vc.name);
					item.textContent = vc.icon;
					container.appendChild(item);
				}
			})
			.catch(() => {
				// Backend not available (e.g., tests) — no-op
			});
	}
}
