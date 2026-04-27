/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * titleBar.ts — top title bar part.
 *
 * Mirrors src/vs/workbench/browser/parts/titlebar/titlebarPart.ts.
 */

import { Disposable } from '../lifecycle.js';

export class TitleBar extends Disposable {
	private _element: HTMLElement | null = null;

	mount(host: HTMLElement): void {
		const el = document.createElement('div');
		el.setAttribute('data-part', 'titlebar');
		el.setAttribute('role', 'banner');
		el.setAttribute('aria-label', 'Title Bar');
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
