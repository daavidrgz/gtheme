window.onload = setListener();

function setListener() {
	document.querySelector('.search-bar').addEventListener('submit', function(e) {
		e.preventDefault();
		googleSearch();
	});
}

function googleSearch() {
	window.location.assign(`http://google.com/search?q=${document.getElementById('search-input').value}`)
}
