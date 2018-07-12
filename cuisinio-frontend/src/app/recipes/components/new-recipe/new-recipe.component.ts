import { Component, OnInit, Output, EventEmitter } from "@angular/core"
import { FormBuilder, FormGroup, FormControl } from "@angular/forms"
import { Recipe, RecipeDTO } from "../../model/recipe.model"
import { RecipeService } from "../../services/recipe.service"

@Component({
    selector: "cuisinio-new-recipe",
    templateUrl: "./new-recipe.component.html",
    styleUrls: ["./new-recipe.component.css"],
})
export class NewRecipeComponent implements OnInit {
    @Output("submit") onSubmitEventEmitter = new EventEmitter<RecipeDTO>()
    form: FormGroup
    steps: string[] = []

    constructor(
        private recipeService: RecipeService,
        private formBuilder: FormBuilder
    ) {
        this.form = this.formBuilder.group({
            name: this.formBuilder.control(null),
            description: this.formBuilder.control(null),
        })
    }

    ngOnInit() {}

    addStep(event) {
        event.stopPropagation()
        const key = "step_" + this.steps.length
        this.steps.push(key)
        this.form.addControl(key + "_name", this.formBuilder.control(null))
        this.form.addControl(
            key + "_description",
            this.formBuilder.control(null)
        )
        this.form.addControl(
            key + "_duration_minutes",
            this.formBuilder.control(null)
        )
    }

    onSubmit() {
        console.log("??", this.form)
        if (this.form.valid) {
            const steps = []
            for (let key of this.steps) {
                let duration = this.form.value[key + "_duration_minutes"]
                duration = duration * 60 * 1000
                steps.push({
                    name: this.form.value[key + "_name"],
                    description: this.form.value[key + "_description"],
                    duration_ms: duration,
                })
            }
            this.recipeService.postRecipe({
                name: this.form.value.name,
                description: this.form.value.description,
                steps: steps,
                graph: {
                    edges: [],
                },
            })
        }
    }
}
