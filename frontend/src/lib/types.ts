export interface Bookmark {
	id: number;
	name: string;
	url: string;
	favicon: string;
	favicon_url: string;
	created: string;
	folder_id: number | null;
	favorite: boolean;
}

export interface Folder {
	id: number;
	name: string;
	parent_id: number | null;
}

export interface FolderNode {
	id: number;
	name: string;
	parent_id: number | null;
	children: FolderNode[];
	bookmarks: Bookmark[];
}

export interface RootItems {
	root_folders: FolderNode[];
	root_bookmarks: Bookmark[];
}
