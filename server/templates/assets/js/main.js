/**
 * @constructor
 * @param {HTMLElement} element
 */

function closeAutosuggest(element) {
    element = document.querySelector('.autosuggest-results')
    setTimeout(() => {
        if (!(document.activeElement.id === 'search-input') && !document.activeElement.id.includes('search-result-') && !(document.activeElement.id === 'search-button') && !(document.activeElement.id === 'clear')) { 
            document.getElementById('autosuggest-results').innerHTML = '';
            element.style.display = 'none'
        }
    }, 100)
}

/**
 * @constructor
 * @param {HTMLElement} element
 * @param {number} inputElement 
 */
function hideSuggest(element, inputElement){
    element = document.querySelector('.autosuggest-results')
    inputElement = document.getElementById('search-input').constructor.length
    if(inputElement <= 2){
        element.style.display = 'none'
    }
}
