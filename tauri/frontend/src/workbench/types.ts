/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

/**
 * workbench/types.ts — shared types for workbench parts.
 */

/** Mirrors the Rust workbench_view_containers command response shape. */
export interface ViewContainer {
	id: string;
	name: string;
	icon: string;
}
