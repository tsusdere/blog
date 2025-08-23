import BasicBreadcrumbs from '../../components/Breadcrumbs.tsx'
const Home = () => {
  return (
    <div class="flex flex-col space-y-135">
      <div class="bg-blue-500 ">
        <BasicBreadcrumbs/>
      </div>
      <div class="bg-black">02</div>
      <div class="bg-black">03</div>
    </div>
  )
};

export default Home;
