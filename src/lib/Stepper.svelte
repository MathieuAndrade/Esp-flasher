<script lang="ts">
  export let step: number = 0;

  let classes = [
    "text-blue-500 flex w-full items-center after:w-full after:h-1 after:border-b after:border-1 after:mx-6 after:border-gray-700",
    "text-slate-200 flex w-full items-center after:w-full after:h-1 after:border-b after:border-1 after:mx-6 after:border-gray-700",
  ];
</script>

<ol
  class="flex items-center w-full text-sm font-medium text-center text-gray-400 px-10"
>
  <li class={step >= 0 ? classes[0] : classes[1]}>
    <svg
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
      fill="currentColor"
      class="w-20 h-20"
    >
      <g>
        <path
          d="M16.19 2H7.81C4.17 2 2 4.17 2 7.81V16.18C2 19.83 4.17 22 7.81 22H16.18C19.82 22 21.99 19.83 21.99 16.19V7.81C22 4.17 19.83 2 16.19 2ZM16 12.75H12.75V16C12.75 16.41 12.41 16.75 12 16.75C11.59 16.75 11.25 16.41 11.25 16V12.75H8C7.59 12.75 7.25 12.41 7.25 12C7.25 11.59 7.59 11.25 8 11.25H11.25V8C11.25 7.59 11.59 7.25 12 7.25C12.41 7.25 12.75 7.59 12.75 8V11.25H16C16.41 11.25 16.75 11.59 16.75 12C16.75 12.41 16.41 12.75 16 12.75Z"
        />
      </g>
    </svg>
  </li>
  <li class={step >= 1 ? classes[0] : classes[1]}>
    <svg
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      stroke="currentColor"
      class="w-20 h-20"
    >
      <path
        d="M19.786 7.806A1.412 1.412 0 0 0 19 7.1V5.576a3.491 3.491 0 0 0-.433-1.655 3.49 3.49 0 0 0-6.271.21 3.06 3.06 0 0 0-.294 1.311L12 19.307a2.021 2.021 0 0 1-.124.76A3.106 3.106 0 0 1 9.01 22a3.127 3.127 0 0 1-2.907-2.185 2.171 2.171 0 0 1-.102-.71v-2.209a1.433 1.433 0 0 0 .771-.672l1.963-3.051A2.68 2.68 0 0 0 9 12.013V4H8V1H3v3H2v8.014a2.693 2.693 0 0 0 .296 1.212l1.918 2.968a1.432 1.432 0 0 0 .786.702v2.25a4.207 4.207 0 0 0 .301 1.512A4.11 4.11 0 0 0 9.01 23a4.109 4.109 0 0 0 3.686-2.327 4.153 4.153 0 0 0 .302-1.503L13 5.434a2.171 2.171 0 0 1 .24-.972 2.486 2.486 0 0 1 4.486-.005A2.4 2.4 0 0 1 18 5.568v1.533a1.412 1.412 0 0 0-.771.675l-1.963 3.051A2.675 2.675 0 0 0 15 11.985V20h1v3h5v-3h1v-8.015a2.689 2.689 0 0 0-.296-1.21zM4 2h3v2H4zm-.834 10.737A1.674 1.674 0 0 1 3 12.014V5h5v7.014a1.71 1.71 0 0 1-.136.67l-1.976 3.073a.44.44 0 0 1-.805-.052zM20 22h-3v-2h3zm1-3h-5v-7.015a1.709 1.709 0 0 1 .136-.67l1.976-3.072a.44.44 0 0 1 .805.052l1.917 2.968a1.673 1.673 0 0 1 .166.722z"
      />
    </svg>
  </li>
  <li
    class={step >= 2
      ? " flex items-center text-blue-500"
      : "flex items-center text-slate-200"}
  >
    <svg
      viewBox="-4 0 16 16"
      xmlns="http://www.w3.org/2000/svg"
      class="w-14 h-14"
      fill="currentColor"
    >
      <path
        d="M-8,1l-2,5h3l-5,9,1-7h-2l1-7h4m1,5h.005M-8,0h-4a1,1,0,0,0-.99.859l-1,7a1,1,0,0,0,.235.8A1,1,0,0,0-13,9h.847l-.837,5.859a1,1,0,0,0,.671,1.089A1.01,1.01,0,0,0-12,16a1,1,0,0,0,.874-.514l4.972-8.951A.987.987,0,0,0-6,6a1,1,0,0,0-1-1H-8.523l1.451-3.629a1,1,0,0,0-.1-.932A1,1,0,0,0-8,0Zm0,2h0Z"
        transform="translate(14)"
      />
    </svg>
  </li>
</ol>
