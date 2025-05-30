
// Initial values
const leftPanelContainer = document.getElementById('left-panel');

// Function for loading new page content
const loadPage = (chapterName) => {
  const filePath = `chapters/${chapterName}.html`;

  fetch(filePath)
    .then(response => {
      if (!response.ok) throw new Error("Page not found");
      return response.text();
    })
    .then(html => {
      document.getElementById('page-mount-point').innerHTML = html;
      Prism.highlightAll();
    })
    .catch(error => {
      console.error(error);
      const mountPoint = document.getElementById('page-mount-point');
      mountPoint.innerHTML = `<p>Error loading ${chapterName}</p>`;
    });
}

loadPage('intro');
