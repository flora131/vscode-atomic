/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * statusBar.ts — bottom status bar part.
 *
 * Mirrors src/vs/workbench/browser/parts/statusbar/statusbarPart.ts.
 */

import { Disposable } from '../lifecycle.js';

export class StatusBar extends Disposable {
	private _element: HTMLElement | null = null;

	mount(host: HTMLElement): void {
		const el = document.createElement('div');
		el.setAttribute('data-part', 'statusbar');
		el.setAttribute('role', 'status');
		el.setAttribute('aria-label', 'Status Bar');
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
