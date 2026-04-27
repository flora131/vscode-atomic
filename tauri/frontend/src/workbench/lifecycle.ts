/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * lifecycle.ts — minimal Disposable base mirroring
 * src/vs/base/common/lifecycle.ts Disposable pattern.
 *
 * Each workbench part extends this to get automatic cleanup
 * of registered child disposables.
 */

export interface IDisposable {
	dispose(): void;
}

/**
 * Minimal Disposable base class.
 * Subclasses call _register() to add child disposables that are
 * automatically disposed when this object is disposed.
 */
export class Disposable implements IDisposable {
	private _disposed = false;
	private readonly _children: IDisposable[] = [];

	get isDisposed(): boolean {
		return this._disposed;
	}

	protected _register<T extends IDisposable>(child: T): T {
		if (this._disposed) {
			child.dispose();
		} else {
			this._children.push(child);
		}
		return child;
	}

	dispose(): void {
		if (this._disposed) {
			return;
		}
		this._disposed = true;
		for (const child of this._children) {
			child.dispose();
		}
		this._children.length = 0;
	}
}
