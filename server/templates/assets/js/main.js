function closeAutosuggest() {
    setTimeout(() => {
        if (!(document.activeElement.id === 'search-input') && !document.activeElement.id.includes('search-result-') && !(document.activeElement.id === 'search-button')) { 
            document.getElementById('autosuggest-results').innerHTML = '';
        }
    }, 100)
}
