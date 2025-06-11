import { v4 as uuidv4 } from 'uuid';

export type FileInfo = {
  name: string;
  path: string;
  extension: string;
  autonomiAddress?: string;
  previewCost?: string;
  actualCost?: string;
  fileSize?: number;

};

export class FileObj {
  private _uuid: string;
  private _name: string;
  private _path: string;
  private _extension: string;
  private _uploadedDate: string;
  private _autonomiAddress?: string;
  private _previewCost?: string;
  private _actualCost?: string;
  private _fileSize?: number;

  constructor(file: FileInfo) {
    this._name = file.name;
    this._path = file.path;
    this._extension = file.extension;
    this._uploadedDate = new Date().toISOString();
    this._uuid = uuidv4();
    this._autonomiAddress = file.autonomiAddress ?? "";
    this._previewCost = file.previewCost ?? "";
    this._actualCost = file.actualCost ?? "";
    this._fileSize = file.fileSize ?? 0;
  }

  get uuid(): string {
    return this._uuid;
  }

  get name(): string {
    return this._name;
  }

  get path(): string {
    return this._path;
  }

  get extension(): string {
    return this._extension;
  }

  get uploadedDate(): string {
    return this._uploadedDate;
  }

  get autonomiAddress(): string | undefined {
    return this._autonomiAddress;
  }

  get previewCost(): string | undefined {
    return this._previewCost;
  }

  get actualCost(): string | undefined {
    return this._actualCost;
  }

  get fileSize(): number | undefined {
    return this._fileSize;
  }

  setAutonomiAddress(address: string): void {
    this._autonomiAddress = address;
  }

  setPreviewCost(cost: string): void {
    this._previewCost = cost;
  }

  setActualCost(cost: string): void {
    this._actualCost = cost;
  }

  setFileSize(size: number): void {
    this._fileSize = size;
  }

  toJSON(): Record<string, any> {
    return {
      uuid: this._uuid,
      name: this._name,
      path: this._path,
      extension: this._extension,
      uploadedDate: this._uploadedDate,
      autonomiAddress: this._autonomiAddress,
      previewCost: this._previewCost,
      actualCost: this._actualCost,
      fileSize: this._fileSize
    };
  }
}
