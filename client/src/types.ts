export interface Table {
  name: string;
}

export interface Services {
  table: string;
  auth: string;
  row: string;
}

export interface Row {
  name: string;
  column_type: string;
  is_nullable: boolean;
  default_value: string;
  is_unique: boolean;
}
