/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * editorArea.ts — central editor area part.
 *
 * Mirrors src/vs/workbench/browser/parts/editor/editorPart.ts.
 * Houses the Monaco editor groups. Monaco integration is a separate task.
 */

import { Disposable } from '../lifecycle.js';

export class EditorArea extends Disposable {
	private _element: HTMLElement | null = null;

	mount(host: HTMLElement): void {
		const el = document.createElement('div');
		el.setAttribute('data-part', 'editor');
		el.setAttribute('role', 'main');
		el.setAttribute('aria-label', 'Editor Area');
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
