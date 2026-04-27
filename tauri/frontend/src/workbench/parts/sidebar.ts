/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * sidebar.ts — sidebar panel part.
 *
 * Mirrors src/vs/workbench/browser/parts/sidebar/sidebarPart.ts.
 */

import { Disposable } from '../lifecycle.js';

export class Sidebar extends Disposable {
	private _element: HTMLElement | null = null;

	mount(host: HTMLElement): void {
		const el = document.createElement('div');
		el.setAttribute('data-part', 'sidebar');
		el.setAttribute('role', 'complementary');
		el.setAttribute('aria-label', 'Sidebar');
		host.appendChild(el);
		this._element = el;

		this._register({
			dispose: () => {
				el.remove();
				this._element = null;
			},
		});
	}
}
