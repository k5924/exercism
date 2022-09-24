package lasagna

// TODO: define the 'PreparationTime()' function
func PreparationTime(layers []string, prepTime int) int {
    if (prepTime == 0){
        return len(layers) * 2
    }
	return len(layers) * prepTime
}

// TODO: define the 'Quantities()' function
func Quantities(layers []string) (int, float64) {
	amountOfNoodles := 0
    amountOfSauce := 0.0
    for i:=0; i<len(layers); i++{
        if (layers[i]=="noodles"){
            amountOfNoodles+=50
        } else if (layers[i]=="sauce"){
        	amountOfSauce+=0.2
        }
    }
	return amountOfNoodles, amountOfSauce
}

// TODO: define the 'AddSecretIngredient()' function
func AddSecretIngredient(friendsList, myList []string){
    secretIngredientIndex := len(friendsList)-1
    lastItemIndex := len(myList)-1
    myList[lastItemIndex] = friendsList[secretIngredientIndex]
}

// TODO: define the 'ScaleRecipe()' function
func ScaleRecipe(quantities []float64, portions int) []float64{
    newRecipe := []float64{}
    for i:=0; i<len(quantities); i++{
        newRecipe = append(newRecipe, quantities[i] * float64(portions)/2)
    }
	return newRecipe
}
