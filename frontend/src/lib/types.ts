export interface Bookmark {
	id: number;
	name: string;
	url: string;
	favicon: string;
	created: string;
	folder_id: number;
}

export interface FolderNode {
	id: number;
	name: string;
	parent_id: number | null;
	children: FolderNode[];
	bookmarks: Bookmark[];
}
