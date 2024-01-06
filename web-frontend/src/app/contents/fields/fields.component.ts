import { Component } from '@angular/core';

import {MetaDataService} from "../../common/api/meta-data.service";
import {Field, FieldValue} from "../../common/model/field";
import {MatTableModule} from "@angular/material/table";
import {ClassificationChild} from "../../common/model/classification";

@Component({
  selector: 'app-fields',
  standalone: true,
  imports: [
    MatTableModule
  ],
  templateUrl: './fields.component.html',
  styleUrl: './fields.component.scss'
})
export class FieldsComponent {
  public displayedColumns: string[] = [

    "name",
    "referenceName",
    "description",
    "type",
    "usage",
    "readOnly",
    "canSortBy",
    "isQueryable",
    "supportedOperations",
    "isIdentity",
    "isPicklist",
    "isPicklistSuggested",
    // "url",
    "isLocked",
  ];

  public values: FieldValue[] = [];

  constructor(private readonly metaDataService: MetaDataService) {

  }

  ngOnInit(): void {
    console.log("ngOnInit");
    this.metaDataService.getMetaDataFields().subscribe((field: Field) => {
      this.values = field.value;
    });
  }

  getSupportedOprationsName(children: ClassificationChild[]) {
    return children.map((c) => { return c.name});
  }
}
