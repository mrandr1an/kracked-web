document.getElementById("blog-search").addEventListener("click", function () {
  window.location.href = "blogsearch";
});

document.getElementById("cv").addEventListener("click", function () {
  window.location.href = "cv";
});

document.getElementById("about-blog").addEventListener("click", function () {
  window.location.href = "blogabout";
});

document.addEventListener("DOMContentLoaded", function () {
  const searchInput = document.getElementById("search-bar");
  const resultsElement = document.getElementById("results");

  searchInput.addEventListener("input", debounce(handleSearch, 300));

  function handleSearch() {
    const keyword = searchInput.value;
    const backendUrl = 'https://kracked.site/blogsearch';

    fetch(backendUrl, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ "value": keyword }),
    })
      .then(response => response.json())
      .then(data => {
	  const complexElement = createComplexElement(data.title, data.desc);
          resultsElement.appendChild(complexElement);
      })
      .catch(error => {
        console.error("Error:", error);
      });
  }

  function debounce(func, delay) {
    let timer;
    return function () {
      const context = this;
      const args = arguments;
      clearTimeout(timer);
      timer = setTimeout(function () {
        func.apply(context, args);
      }, delay);
    };
  }

  function createComplexElement(title, description) {
      if(title != "No Results"){
    const outerDiv = document.createElement("div");
    outerDiv.className = "col-md-6 col-lg-4";
    outerDiv.id = "blog-card";

    const innerDiv = document.createElement("div");
    innerDiv.className = "card mb-3 rounded";

    const cardBodyDiv = document.createElement("div");
    cardBodyDiv.className = "card-body";

    const titleElement = document.createElement("h5");
    titleElement.className = "card-title";
    titleElement.textContent = title;

    const descriptionElement = document.createElement("p");
    descriptionElement.className = "card-text";
    descriptionElement.textContent = description;

    const linkElement = document.createElement("a");
    linkElement.href = "#";
    linkElement.className = "btn btn-primary";
    linkElement.textContent = "Click this card for more";

    cardBodyDiv.appendChild(titleElement);
    cardBodyDiv.appendChild(descriptionElement);
    cardBodyDiv.appendChild(linkElement);

    innerDiv.appendChild(cardBodyDiv);

    outerDiv.appendChild(innerDiv);

    return outerDiv;
      }
      else
      {

	  const outerDiv = document.createElement("div");
	  return outerDiv;
      }
  }

});
