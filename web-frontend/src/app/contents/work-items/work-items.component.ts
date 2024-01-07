import {AfterViewInit, Component, OnInit, signal, ViewChild} from '@angular/core';
import {DataStoreService} from "../../common/api/data-store.service";
import {WorkItem} from "../../common/model/workitems";
import {MatTableDataSource, MatTableModule} from "@angular/material/table";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatPaginator, MatPaginatorModule} from "@angular/material/paginator";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {MetaDataService} from "../../common/api/meta-data.service";
import {Field, FieldValue, SupportedOperation} from "../../common/model/field";
import {
  CdkDragDrop,
  moveItemInArray,
  transferArrayItem,
  CdkDrag,
  CdkDropList, CdkDropListGroup,
} from '@angular/cdk/drag-drop';
import {MatListModule} from "@angular/material/list";


export interface FieldViewModel {
  name: string;
  referenceName: string;
  description: string;
}

@Component({
  selector: 'app-work-items',
  standalone: true,
  imports: [
    MatTableModule,
    MatExpansionModule,
    MatPaginatorModule,
    MatFormFieldModule,
    MatInputModule,
    CdkDropList,
    CdkDrag,
    CdkDropListGroup,
    MatListModule
  ],
  templateUrl: './work-items.component.html',
  styleUrl: './work-items.component.scss'
})
export class WorkItemsComponent implements AfterViewInit, OnInit {
  public displayedColumns: string[] = [
    "id",
    "rev",
    "fields",
  ];
  public values: WorkItem[] = [];

  public fields: FieldValue[] = [];

  public selectedFields: FieldViewModel[] = [];

  public unselectedFields: FieldViewModel[] = [];

  dataSource = new MatTableDataSource<WorkItem>(this.values);

  @ViewChild(MatPaginator) paginator!: MatPaginator;
  toggleAllRows = signal<null | null>(null);

  constructor(private readonly dataStoreService: DataStoreService, private readonly metaDataService: MetaDataService) {

    // this.dataSource.filterPredicate = (data: WorkItem, filter: string) => {
    //   // dataはフィルタリングされる行、filterはフィルター条件
    //   // ここで動的なフィルタリングロジックを実装
    //   return data.yourDynamicField.includes(filter);
    // };

  }

  ngAfterViewInit() {
    console.log("AfterViewInit")
    // this.dataSource.paginator = this.paginator;
  }


  ngOnInit(): void {
    console.log("ngOnInit");
    this.metaDataService.getMetaDataFields().subscribe((field: Field) => {
      this.fields = field.value;

      for(let f of this.fields){
        this.unselectedFields.push({
          name: f.name,
          referenceName: f.referenceName,
          description: f.description
        });
      }
    })

    this.dataStoreService.getWorkItems().subscribe((workItems: WorkItem[]) => {
      this.values = workItems;
      this.dataSource = new MatTableDataSource<WorkItem>(this.values);
      this.dataSource.paginator = this.paginator;
      // this.dataSource.data = this.values;
      console.log(this.values);
    });
  }

  applyFilter(event: Event) {
    const filterValue = (event.target as HTMLInputElement).value;
    this.dataSource.filter = filterValue.trim().toLowerCase();
  }


}
