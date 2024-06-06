document.addEventListener('DOMContentLoaded', () => {
  fetchBooks();

  function fetchBooks() {
      fetch('/books')
          .then(response => response.json())
          .then(books => {
              const bookList = document.getElementById('book-list');
              bookList.innerHTML = '';
              books.forEach(book => {
                  const bookItem = document.createElement('div');
                  bookItem.classList.add('book-item');
                  bookItem.setAttribute('draggable', 'true');
                  bookItem.setAttribute('data-id', book.id);
                  bookItem.innerHTML = `
                      <h2>${book.title}</h2>
                      <p>${book.author}</p>
                  `;
                  bookList.appendChild(bookItem);
              });

              addDragAndDrop();
          });
  }

  function addDragAndDrop() {
      const bookItems = document.querySelectorAll('.book-item');

      bookItems.forEach(item => {
          item.addEventListener('dragstart', handleDragStart);
          item.addEventListener('dragover', handleDragOver);
          item.addEventListener('drop', handleDrop);
          item.addEventListener('dragend', handleDragEnd);
      });

      function handleDragStart(e) {
          e.dataTransfer.setData('text/plain', e.target.getAttribute('data-id'));
          e.target.classList.add('dragging');
      }

      function handleDragOver(e) {
          e.preventDefault();
          e.target.classList.add('drag-over');
      }

      function handleDrop(e) {
          e.preventDefault();
          const draggedId = e.dataTransfer.getData('text/plain');
          const targetId = e.target.getAttribute('data-id');
          
          if (draggedId !== targetId) {
              const bookList = document.getElementById('book-list');
              const draggedElement = document.querySelector(`.book-item[data-id='${draggedId}']`);
              const targetElement = document.querySelector(`.book-item[data-id='${targetId}']`);
              
              bookList.insertBefore(draggedElement, targetElement.nextSibling);
          }

          e.target.classList.remove('drag-over');
      }

      function handleDragEnd(e) {
          e.target.classList.remove('dragging');
      }
  }
});
