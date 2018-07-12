import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { HttpModule } from '@angular/http';
import { FormsModule } from '@angular/forms';

import { RecipeModule } from './recipes/recipes.module';
import { AppComponent } from './app.component';

const appRoutes: Routes = [
    {
        path: '',
        redirectTo: 'recipes',
        pathMatch: 'full'
    }
];

@NgModule({
    declarations: [
        AppComponent
    ],
    imports: [
        BrowserModule
        , RouterModule.forRoot(appRoutes)
        , RecipeModule
        , HttpModule
        , FormsModule
    ],
    providers: [],
    bootstrap: [AppComponent]
})
export class AppModule { }

